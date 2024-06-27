<script lang="ts">
		import ThemeSwitcher from "./theme-switcher.svelte";
		import logo from '$lib/assets/images/logo.png'
</script>

<nav class="sticky top-0 flex h-16 w-full items-center justify-between border-b-2 bg-transparent px-6 md:px-16 backdrop-blur">
	<div class="flex  gap-2 items-center justify-center">
		<img src={logo} alt="Paws and Claws" class="h-8 w-8" />
		<h1 class="font-semibold text-2xl">Paws and Claws</h1>
	</div>
	<div class="flex items-center justify-center gap-2">
		<ThemeSwitcher />
	</div>

</nav>