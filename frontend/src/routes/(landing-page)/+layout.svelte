<script lang="ts">
	import Navbar from "$lib/components/navbar.svelte";

    let {children} = $props()
</script>

<main class="flex min-h-screen flex-col px-1">
    <div class="">
        <Navbar/>
    </div>

    <div>{@render children()}</div>
</main>