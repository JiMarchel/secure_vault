<script lang="ts">
	import '../app.css';
	import witchHat from '$lib/assets/witch-hat.png';
	import { ModeWatcher } from 'mode-watcher';
	import SvelteSeo from 'svelte-seo';

	let { children } = $props();
	let title = 'Secure Vault - Secure your secrets';
	let description =
		'Secure Vault is a user-friendly secrets management tool designed to help you securely store and manage your sensitive information.';
</script>

<svelte:head>
	<link rel="icon" href={witchHat} />
</svelte:head>

<ModeWatcher />
<SvelteSeo
	{title}
	{description}
	applicationName="Secure Vault"
	keywords="secure vault, secrets management, password manager, encryption, secure storage"
	openGraph={{
		title,
		description,
		type: 'website',
		site_name: 'Secure Vault'
	}}
	twitter={{
		card: 'summary',
		site: '@jimarchel',
		title,
		description
	}}
/>
{@render children?.()}
