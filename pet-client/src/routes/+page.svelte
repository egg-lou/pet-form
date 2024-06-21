<script lang="ts">
	import {ArrowRight} from 'phosphor-svelte';

	const links: {name: string; href: string, description: string}[] = [
	{name: 'Add a record', href: '/registration', description: 'Add a new pet/owner and its information'},
	{name: 'View Records', href: '/records', description: 'Access all the complete list of registered owners and their pets'},
	{name: 'View All Pet History', href: '/pets', description: 'Review detailed history records of all pets'},
]
</script>

<div class="container h-full mx-auto flex justify-center items-center">
	<div class="space-y-10 text-center flex flex-col items-center w-full">
		<h1 class="text-4xl">Welcome to Paws and Claws!</h1>
		<div class="flex gap-4 w-full justify-evenly">
			{#each links as {name, href, description}}
				<div class="card card-hover p-4 flex flex-col justify-between gap-10 w-[400px] h-full">
					<div class="h-[50px]">
						<p>{description}</p>
					</div>
					<div class="card-footer flex gap-3 justify-end items-center">
						<a href={href}>{name}</a>
						<ArrowRight class="w-4 h-4" />
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>

<style lang="postcss">
	figure {
		@apply flex relative flex-col;
	}
	figure svg,
	.img-bg {
		@apply w-64 h-64 md:w-80 md:h-80;
	}
	.img-bg {
		@apply absolute z-[-1] rounded-full blur-[50px] transition-all;
		animation: pulse 5s cubic-bezier(0, 0, 0, 0.5) infinite,
			glow 5s linear infinite;
	}
	@keyframes glow {
		0% {
			@apply bg-primary-400/50;
		}
		33% {
			@apply bg-secondary-400/50;
		}
		66% {
			@apply bg-tertiary-400/50;
		}
		100% {
			@apply bg-primary-400/50;
		}
	}
	@keyframes pulse {
		50% {
			transform: scale(1.5);
		}
	}
</style>
