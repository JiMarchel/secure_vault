<script lang="ts">
	import ToggleTheme from './toggle-theme.svelte';
	import Button from './ui/button/button.svelte';
	import witchHat from '$lib/assets/witch-hat.png';
	import { page } from '$app/state';

	const currentPath = $derived(page.url.pathname);

</script>

<nav class="flex justify-between border-b py-2 shadow-md">
	<div class="flex items-center gap-2 font-bold">
		<img src={witchHat} alt="logo" class="h-8 w-8" />
		<h1 class="text-3xl">SecureVault</h1>
	</div>
	<div class="flex items-center gap-4">
		<div class="gap-1 font-medium">
			<a href="/sign-in">
				<Button variant={currentPath === '/sign-in' ? 'default' : 'outline'}>Sign In</Button>
			</a>
			<a href="/sign-up">
				<Button variant={currentPath === '/sign-up' ? 'default' : 'outline'}>Sign Up</Button>
			</a>
		</div>
		<!-- <ToggleTheme/> -->
	</div>
</nav>
