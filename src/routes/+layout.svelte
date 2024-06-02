<script>
    import "../app.css";
</script>

<slot/>

<style>
    @import url('/assets/fonts/montserrat.css');

    :global(html) {
        font-family: "Montserrat", sans-serif;
        background-color: #1B1B1D;
        color: white;
    }

    :global(:root) {
        --date-picker-background: #151516;
	    --date-picker-foreground: #f7f7f7;
    }

    :global(.date-time-picker) {
        border-radius: 7px !important;
        background-color: rgba(18,18,18,0.6) !important;
        backdrop-filter: blur(30px);
        -webkit-backdrop-filter: blur(30px);
    }

    :global(.date-time-picker .dropdown select) {
        border-radius: 5px !important;
        background-color: rgba(18,18,18,0.1) !important;
        backdrop-filter: blur(30px);
        -webkit-backdrop-filter: blur(30px);
    }
</style>
