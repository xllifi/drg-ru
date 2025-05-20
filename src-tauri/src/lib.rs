use std::{
  fs::{self, File},
  io::{self, BufReader, BufWriter},
  path::{Path, PathBuf},
  sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
  },
};

use path_slash::PathExt;
use rayon::{
  iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator},
  ThreadPoolBuilder,
};
use regex::Regex;
use repak::{Compression, Version};
use serde::Serialize;
use tauri::ipc::Channel;

// TODO: add downloading mod files

async fn download() {
  
}

#[tauri::command]
async fn unpack(
  in_pak_path: String,
  out_dir_path: String,
  on_event: Channel<ProgressEvent>,
) -> Result<(), String> {
  let path = PathBuf::from(&in_pak_path);
  println!("{:?}", path);
  let file = File::open(&path).map_err(|err| format!("{err}"))?;
  let mut reader = BufReader::new(&file);

  let builder = repak::PakBuilder::new();
  let pak = builder
    .reader(&mut reader)
    .map_err(|err| format!("{err}"))?;

  match fs::create_dir(&out_dir_path) {
    Ok(_) => Ok(()),
    Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
    Err(e) => Err(e),
  }
  .map_err(|err| format!("{err}"))?;

  struct UnpackEntry {
    entry_path: String,
    out_path: PathBuf,
    out_dir: PathBuf,
  }

  let entries = pak.files().into_iter().map(|entry_path| {
    let out_path = PathBuf::from(&out_dir_path).join(&entry_path);
    let out_dir = out_path.parent().expect("will be a file").to_path_buf();

    UnpackEntry {
      entry_path,
      out_path,
      out_dir,
    }
  });

  let current = Arc::new(AtomicU32::new(0));
  let length = entries.len() as u32;

  entries
    .par_bridge()
    .try_for_each_init(
      || File::open(&in_pak_path),
      |file, entry| -> Result<(), String> {
        let current = current.fetch_add(1, Ordering::SeqCst);
        if current % 64 == 0 {
          on_event
            .send(ProgressEvent {
              current,
              total: length.clone(),
            })
            .map_err(|e| format!("{e}"))?;
        }

        fs::create_dir_all(&entry.out_dir).map_err(|e| format!("{e}"))?;
        pak
          .read_file(
            &entry.entry_path,
            &mut BufReader::new(
              file
                .as_ref()
                .map_err(|e| format!("error reading pak: {e}"))?,
            ),
            &mut File::create(&entry.out_path).map_err(|e| format!("{e}"))?,
          )
          .map_err(|e| format!("{e}"))
      },
    )
    .map_err(|err| format!("{err}"))?;

  Ok(())
}

#[tauri::command]
async fn insert_files(
  in_dir_path: String,
  out_dir_path: String,
  on_event: Channel<ProgressEvent>,
) -> Result<(), String> {
  let mut in_file_paths = vec![];
  collect_files(&mut in_file_paths, Path::new(&in_dir_path)).map_err(|err| format!("{err}"))?;

  let re = Regex::new(r"\./run/in").map_err(|err| format!("{err}"))?;

  let current = Arc::new(AtomicU32::new(0));
  let length = in_file_paths.len() as u32;

  in_file_paths
    .par_iter()
    .try_for_each(|file_path| -> Result<(), String> {
      if file_path.is_dir() {
        return Ok(());
      }
      let current = current.fetch_add(1, Ordering::SeqCst);
      if current % 64 == 0 {
        on_event
          .send(ProgressEvent {
            current,
            total: length.clone(),
          })
          .map_err(|e| format!("{e}"))?;
      }

      let dirless_path = re.replace(
        file_path
          .to_str()
          .ok_or("failed to convert path to string")?,
        "",
      );

      let in_full = PathBuf::from(format!("{}{}", &in_dir_path, &dirless_path));
      let out_full = PathBuf::from(format!("{}{}", &out_dir_path, &dirless_path));

      if let Some(parent) = out_full.parent() {
        fs::create_dir_all(parent).map_err(|err| format!("{err}"))?;
      }
      fs::copy(in_full, out_full).map_err(|err| format!("{err}"))?;

      Ok(())
    })?;

  Ok(())
}

#[tauri::command]
async fn repack(
  in_dir_path: String,
  out_pak_path: String,
  on_event: Channel<ProgressEvent>,
) -> Result<(), String> {
  let input_path = Path::new(&in_dir_path);
  if !input_path.is_dir() {
    return Err("Not a directory!".into());
  }

  let mut paths = vec![];
  collect_files(&mut paths, &input_path).map_err(|err| format!("{err}"))?;

  let mut pak = repak::PakBuilder::new()
    .compression(Some(Compression::Zlib))
    .writer(
      BufWriter::new(File::create(&out_pak_path).map_err(|err| format!("{err}"))?),
      Version::V11,
      "../../../".into(),
      None,
    );

  let iter = paths.iter();

  let current = Arc::new(AtomicU32::new(0));
  let length = iter.len() as u32;

  let mut result = None;
  let result_ref = &mut result;
  rayon::in_place_scope(|scope| -> Result<(), repak::Error> {
    let (tx, rx) = std::sync::mpsc::sync_channel(0);
    let entry_builder = pak.entry_builder();

    scope.spawn(move |_| {
      *result_ref = Some(
        iter
          .par_bridge()
          .try_for_each(|p| -> Result<(), repak::Error> {
            let rel = &p
              .strip_prefix(input_path)
              .expect("file not in input directory")
              .to_slash()
              .expect("failed to convert to slash path");

            let current = current.fetch_add(1, Ordering::SeqCst);
            if current % 64 == 0 {
              on_event
                .send(ProgressEvent {
                  current,
                  total: length.clone(),
                })
                .map_err(|e| repak::Error::Other(format!("{e}")))?;
            }

            let entry = entry_builder.build_entry(true, std::fs::read(p)?)?;

            tx.send((rel.to_string(), entry)).unwrap();
            Ok(())
          }),
      );
    });

    for (path, entry) in rx {
      pak.write_entry(path, entry)?;
    }
    Ok(())
  })
  .map_err(|err| format!("{err}"))?;
  result.unwrap().map_err(|err| format!("{err}"))?;

  pak.write_index().map_err(|err| format!("{err}"))?;

  println!("Packed {} files to {}", paths.len(), out_pak_path);

  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  ThreadPoolBuilder::new()
    .num_threads(4)
    .build_global()
    .expect("Failed to configure global thread pool");

  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![unpack, insert_files, repack])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn collect_files(paths: &mut Vec<PathBuf>, dir: &Path) -> io::Result<()> {
  for entry in fs::read_dir(dir)? {
    let entry = entry?;
    let path = entry.path();
    if path.is_dir() {
      collect_files(paths, &path)?;
    } else {
      paths.push(entry.path());
    }
  }
  Ok(())
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event")]
struct ProgressEvent {
  current: u32,
  total: u32,
}
