<script lang="ts">
  import { goto } from '$app/navigation'
  import { onMount } from 'svelte'

  // Redirect to /process automatically
  onMount(() => {
    goto('/process')
  })
</script>

<svelte:head>
  <title>AI File Processing</title>
</svelte:head>

<div class="container">
  <h1>AI File Processing</h1>
  <p>Redirecting to file processing...</p>
  <a href="/process">Click here if not redirected</a>
</div>

<style>
  .container {
    max-width: 600px;
    margin: 100px auto;
    text-align: center;
    padding: 20px;
  }

  h1 {
    color: #333;
    border-bottom: 2px solid #ff3e00;
    padding-bottom: 10px;
    display: inline-block;
  }

  p {
    color: #666;
    margin: 20px 0;
  }

  a {
    color: #ff3e00;
    text-decoration: none;
  }

  a:hover {
    text-decoration: underline;
  }
</style>
