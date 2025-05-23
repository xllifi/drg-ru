<script>
  import NotificationsFeed from '$lib/components/NotificationsFeed.svelte';
  import ProgressBar from '$lib/components/ProgressBar.svelte';

	let { children } = $props();
</script>

{@render children()}
<ProgressBar />
<NotificationsFeed />