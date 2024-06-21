<script lang="ts">
	import { page } from "$app/stores";
  const {status, error} = $page

	console.log(status, error.message)
</script>
<div class="w-full h-full flex flex-col justify-center items-center gap-4">
<h1 class="text-9xl">{status}</h1>
<p class="text-4xl">{error.message}</p>
<a href="/" class="btn variant-filled-error">Return to Home</a>
</div>
