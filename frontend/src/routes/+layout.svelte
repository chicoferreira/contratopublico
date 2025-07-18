<script>
    let { children } = $props();
    import "../app.css";
</script>

<div class="navbar bg-base-100 shadow-sm">
    <div class="navbar-start"></div>
    <div class="navbar-center">
        <a class="btn btn-ghost text-xl" href="/">contratoPúblico</a>
    </div>
    <div class="navbar-end">
        <!-- TODO: github link & report issue -->
    </div>
</div>

<main class="min-h-screen flex">
    <div class="container mx-auto px-4">
        {@render children()}
    </div>
</main>
