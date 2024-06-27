<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
 import Navbar from '@/components/ui/navbar/navbar.svelte';
</script>

<ModeWatcher />
<Navbar />
<slot />

<style></style>
