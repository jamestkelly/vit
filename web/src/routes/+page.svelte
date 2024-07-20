<script lang="ts">
  import { onMount } from 'svelte';

  export let data = {
    hello: ""
  };

  const url = "greet?name=onMount";
  let greeting = "";

  async function getGreeting() {
    const opts = {
      method: "get",
    };
    const response = await fetch(url, opts);
    const hello = await response.text();
    console.log("hello:", hello);

    greeting = hello;
  }

  onMount(() => getGreeting());
</script>

<h1>{greeting}</h1>

<h1>{data.hello}</h1>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
