<script>
    import { onMount } from 'svelte';
    import { getClient } from '@tauri-apps/api/http';
    import { open } from '@tauri-apps/api/shell';
    import { fade } from 'svelte/transition';
    import { appWindow } from '@tauri-apps/api/window';
    import { DatePicker } from 'date-picker-svelte';
    import { PUBLIC_API_KEY } from '$env/static/public';
    import { isOffline } from 'svelte-connection-status';
    import { getVersion } from '@tauri-apps/api/app';

    let appVersion;

    let apod, preview;
    let custom_date = new Date(); // Custom Selected Date
    let select_date; // Show Date Picker

    // Get Youtube Video Thumbnail
    function getVideoThumbnail(url) {
        var videoCode = null;
    
        if (url.includes('youtube.com/watch?v=')) {
            var params = new URLSearchParams(new URL(url).search);
            videoCode = params.get('v');
        } else if (url.includes('youtu.be/')) {
            videoCode = url.split('youtu.be/')[1];
        } else if (url.includes('youtube.com/embed/')) {
            videoCode = url.split('youtube.com/embed/')[1].split('?')[0];
        } else {
            return null;
        }
        
        return "https://img.youtube.com/vi/" + videoCode + "/hqdefault.jpg";
    }

    // Pretify Date
    function FormatDate(date) {
        const new_date = new Date(date);
        const options = { weekday: 'long', year: 'numeric', month: 'short', day: 'numeric' };
        return new_date.toLocaleDateString('ae-ae', options)
    }

    // Close Image Preview
    function closePreview() {
        preview = null;
        document.body.style.overflow = "auto";
    }

    // Fetch APOD
    async function getAPOD(cus_date) {
        closePreview();
        apod = false;
        let url;
        if (cus_date) {
            url = `https://api.nasa.gov/planetary/apod?api_key=${PUBLIC_API_KEY}&date=` + custom_date.toISOString().split('T')[0];
        } else {
            url = `https://api.nasa.gov/planetary/apod?api_key=${PUBLIC_API_KEY}`
        }
        const client = await getClient();
        const result = await client.get(url);
        apod = result.data ?? {};
    } 

    // Right Click Menu
    let pos = { x: 0, y: 0 }
    let menu = { h: 0, y: 0 }
    let browser = { h: 0, y: 0 }
    let showMenu = false;

    function rightClickContextMenu(e){
        showMenu = true
        browser = {
            w: window.innerWidth,
            h: window.innerHeight
        };
        pos = {
            x: e.clientX,
            y: e.clientY
        };
        if (browser.h -  pos.y < menu.h)
            pos.y = pos.y - menu.h
        if (browser.w -  pos.x < menu.w)
            pos.x = pos.x - menu.w
    }
    function onPageClick(e){
        showMenu = false;
    }
    function getContextMenuDimension(node){
        let height = node.offsetHeight
        let width = node.offsetWidth
        menu = {
            h: height,
            w: width
        }
    }
    let menuItems = [
        {
            'name': 'reload-apod',
            'onClick': getAPOD,
            'displayText': "Reload",
        }
    ]
    // End Right Click Menu

    onMount(async () => {
        appVersion = await getVersion(); // Get APP Version
        await getAPOD(false); // Load APOD On Startup
    });
</script>

<div data-tauri-drag-region class="sticky top-0 left-0 w-full py-2 bg-[#242526] flex justify-between items-center px-4 z-50 select-none cursor-default">
    <div class="flex justify-center items-center gap-2 group">
        <p class="font-medium">APOD</p>
        <div class:opacity-100={select_date} class="group-hover:opacity-100 opacity-0 duration-200 gap-2 flex items-center justify-center text-gray-400">
            <button on:click={async () => {await getAPOD(custom_date)}} title="Reload" class="hover:text-white duration-200">
                <svg xmlns="http://www.w3.org/2000/svg" width="0.9rem" height="0.9rem" viewBox="0 0 20 20"><path fill="currentColor" d="M14.66 15.66A8 8 0 1 1 17 10h-2a6 6 0 1 0-1.76 4.24zM12 10h8l-4 4z"/></svg>
            </button>

            <div class="relative flex items-center">
                <button on:click={() => {if (select_date) {select_date = false} else {select_date = true}}} class="hover:text-white duration-200">
                    <svg xmlns="http://www.w3.org/2000/svg" width="0.9rem" height="0.9rem" viewBox="0 0 24 24"><g fill="none"><path fill="currentColor" d="M2 9c0-1.886 0-2.828.586-3.414C3.172 5 4.114 5 6 5h12c1.886 0 2.828 0 3.414.586C22 6.172 22 7.114 22 9c0 .471 0 .707-.146.854C21.707 10 21.47 10 21 10H3c-.471 0-.707 0-.854-.146C2 9.707 2 9.47 2 9m0 9c0 1.886 0 2.828.586 3.414C3.172 22 4.114 22 6 22h12c1.886 0 2.828 0 3.414-.586C22 20.828 22 19.886 22 18v-5c0-.471 0-.707-.146-.854C21.707 12 21.47 12 21 12H3c-.471 0-.707 0-.854.146C2 12.293 2 12.53 2 13z"/><path stroke="currentColor" stroke-linecap="round" stroke-width="2" d="M7 3v3m10-3v3"/></g></svg>
                </button>
                {#if select_date}
                <div transition:fade={{ duration: 150 }} class="absolute top-5">
                    <DatePicker bind:value={custom_date} min={new Date("June 16 1995")} max={new Date} format="YYYY-MM-DD" />
                    <button on:click={async () => {getAPOD(custom_date); select_date = false;}} class="duration-200 text-gray-300 hover:text-white text-sm px-4 py-2 rounded-lg font-medium mt-2 border border-[#2c2f36]" style="background-color: rgba(18,18,18, 0.6); backdrop-filter: blur(30px); -webkit-backdrop-filter: blur(30px);">Confirm Date</button>
                </div>
                {/if}
            </div>
            
        </div>
    </div>
    

    <div class="flex flex-nowrap justify-center items-center h-full gap-3">
        <button on:click={() => {appWindow.minimize()}}>
            <svg xmlns="http://www.w3.org/2000/svg" width="1.2rem" height="1.2rem" viewBox="0 0 24 24" class="mt-0.5"><path fill="currentColor" d="M19 13H5v-2h14z"/></svg>
        </button>
        <!-- <button on:click={() => {appWindow.toggleMaximize()}}>
            <svg xmlns="http://www.w3.org/2000/svg" width="1rem" height="1rem" viewBox="0 0 24 24"><path fill="currentColor" d="M19 3H5c-1.11 0-2 .89-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2m0 2v14H5V5z"/></svg>
        </button> -->
        <button on:click={() => {appWindow.close()}}>
            <svg xmlns="http://www.w3.org/2000/svg" width="1.2rem" height="1.2rem" viewBox="0 0 24 24" class="mt-[1px]"><path fill="currentColor" d="M6.4 19L5 17.6l5.6-5.6L5 6.4L6.4 5l5.6 5.6L17.6 5L19 6.4L13.4 12l5.6 5.6l-1.4 1.4l-5.6-5.6z"/></svg>
        </button>
    </div>
</div>

{#if $isOffline && !apod}
<section class="w-full bg-red-600 p-4 flex items-center justify-start gap-2">
    <svg xmlns="http://www.w3.org/2000/svg" width="1.2rem" height="1.2rem" viewBox="0 0 24 24"><path fill="currentColor" d="m6.35 15.35l-2.1-2.15q1.55-1.55 3.55-2.375T12 10t4.213.838t3.537 2.412l-2.1 2.1q-1.125-1.125-2.588-1.737T12 13t-3.062.613T6.35 15.35M2.1 11.1L0 9q2.375-2.425 5.488-3.713T12 4t6.513 1.288T24 9l-2.1 2.1q-1.975-1.975-4.538-3.037T12 7T6.637 8.063T2.1 11.1M12 21l-3.525-3.55q.7-.7 1.613-1.075T12 16t1.913.375t1.612 1.075z"/></svg>
    <p class="text-white font-medium text-sm">Unable to connect to the internet. Please check your internet connection.</p>
</section>
{/if}

<main class="flex justify-center items-center w-full">
    <section class="max-w-5xl">
    {#if apod}
    {#if apod.media_type == "video"}
    <div class="w-full h-72  bg-contain bg-no-repeat bg-center flex justify-center items-center" style="background-image: url('{getVideoThumbnail(apod.url)}'); background-color: rgba(18,18,18,0.6);">
        <button on:click={async () => {await open(apod.url);}} class="px-4 py-2 rounded-lg font-medium flex items-center justify-center" style="background-color: rgba(18,18,18,0.5); backdrop-filter: blur(30px); -webkit-backdrop-filter: blur(30px);">
            <svg xmlns="http://www.w3.org/2000/svg" width="1.2rem" height="1.2rem" class="mr-2" viewBox="0 0 24 24"><path fill="currentColor" d="M21.409 9.353a2.998 2.998 0 0 1 0 5.294L8.597 21.614C6.534 22.737 4 21.277 4 18.968V5.033c0-2.31 2.534-3.769 4.597-2.648z"/></svg>
            Play Video
        </button>
    </div>
    {:else if apod.media_type == "image"}
    <div class="w-full h-72 bg-cover bg-no-repeat bg-center flex justify-center items-center group duration-200" style="background-image: url('{apod.url}'); background-color: rgba(18,18,18,0.6);">
        <button on:click={async () => {preview = apod.hdurl ?? apod.url; document.body.style.overflow = "hidden";}} class="group-hover:opacity-100 opacity-0 duration-200 px-4 py-2 rounded-lg font-medium flex items-center justify-center" style="background-color: rgba(18,18,18,0.5); backdrop-filter: blur(30px); -webkit-backdrop-filter: blur(30px);">
            <svg xmlns="http://www.w3.org/2000/svg" width="1.2rem" height="1.2rem" class="mr-2" viewBox="0 0 16 16"><path fill="currentColor" d="M1.75 10a.75.75 0 0 1 .75.75v2.5c0 .138.112.25.25.25h2.5a.75.75 0 0 1 0 1.5h-2.5A1.75 1.75 0 0 1 1 13.25v-2.5a.75.75 0 0 1 .75-.75m12.5 0a.75.75 0 0 1 .75.75v2.5A1.75 1.75 0 0 1 13.25 15h-2.5a.75.75 0 0 1 0-1.5h2.5a.25.25 0 0 0 .25-.25v-2.5a.75.75 0 0 1 .75-.75M2.75 2.5a.25.25 0 0 0-.25.25v2.5a.75.75 0 0 1-1.5 0v-2.5C1 1.784 1.784 1 2.75 1h2.5a.75.75 0 0 1 0 1.5ZM10 1.75a.75.75 0 0 1 .75-.75h2.5c.966 0 1.75.784 1.75 1.75v2.5a.75.75 0 0 1-1.5 0v-2.5a.25.25 0 0 0-.25-.25h-2.5a.75.75 0 0 1-.75-.75"/></svg>
            View
        </button>
    </div>
    {/if}
    {#if apod?.code == "404"}
    <div class="p-4 mt-8">
        <p class="text-2xl font-medium">No APOD found for this date.</p>
        <p class="text-gray-300">Please try again later.</p>
    </div>
    {:else}
    <div class="p-4">
        <p class="text-2xl font-medium cursor-text select-text">{apod.title ?? ""}</p>
        <p class="mt-1 text-sm text-gray-300">{FormatDate(apod.date ?? Date.now())} {#if apod.copyright}- {apod.copyright}{/if}</p>
        <p class="mt-2 text-gray-200 text-sm select-text cursor-text">{apod.explanation ?? ""}</p>
    </div>
    {/if}

    {#if preview}
    <div transition:fade={{ duration: 200 }} on:keydown={closePreview} on:click={closePreview} role="button" tabindex="0" class="top-0 left-0 fixed bg-black/80 flex justify-center items-center w-screen h-screen pt-16 p-10 z-5">
        <img src="{preview}" class="max-w-full max-h-full object-contain" alt="APOD Preview"/>
    </div>
    {/if}

    {:else}
    <div class="flex w-full flex-col justify-center items-center pt-20">
        <svg xmlns:svg="http://www.w3.org/2000/svg" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.0" width="64px" height="64px" viewBox="0 0 128 128" xml:space="preserve"><g><path d="M64 16.5A66.53 66.53 0 0 0 .26 64a63.75 63.75 0 0 1 127.5 0h-.02A66.53 66.53 0 0 0 64 16.5z" fill="#ffffff"/><animateTransform attributeName="transform" type="rotate" from="0 64 64" to="360 64 64" dur="1800ms" repeatCount="indefinite"></animateTransform></g></svg>
        <p class="font-medium text-lg mt-6">Loading...</p>
        <p class="text-gray-300 text-sm">APOD Desktop {appVersion}</p>
    </div>
    {/if}
    </section>
</main>

{#if showMenu}
<nav use:getContextMenuDimension style="position: absolute; top:{pos.y}px; left:{pos.x}px; z-index: 100;">
    <div class="navbar" id="navbar">
        <ul>
            {#each menuItems as item}
                {#if item.name == "hr"}
                    <hr>
                {:else}
                    <li><button on:click={item.onClick}><i class={item.class}></i>{item.displayText}</button></li>
                {/if}
            {/each}
        </ul>
    </div>
</nav>
{/if}

<svelte:window on:contextmenu|preventDefault={rightClickContextMenu} 
on:click={onPageClick} />

<style>
    .navbar{
        display: inline-flex;
        border: 1px #2c2f36 solid;
        background-color: rgba(18,18,18,0.6) !important;
        backdrop-filter: blur(30px);
        -webkit-backdrop-filter: blur(30px);
        border-radius: 7px;
        overflow: hidden;
        flex-direction: column;
    }
    .navbar ul{
        margin: 6px;
    }
    ul li{
        display: block;
        list-style-type: none;
    }
    ul li button{
        font-size: 14px;
        color: #d7d7d7;
        width: 100%;
        text-align: left;
        transition: 200ms;
        padding: 0px 10px 0px 10px;
    }
    ul li button:hover{
        color: #ffffff;
    }
    hr{
        border: none;
        border-bottom: 1px solid #ccc;
        margin: 5px 0px;
    }
</style>