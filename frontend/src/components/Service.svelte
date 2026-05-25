<script>
    import { onMount, onDestroy } from "svelte";
    import {
        fetch_services,
        start_service,
        delete_service,
        stop_service,
        check_service,
    } from "../API.js";

    let services = $state([]);
    let intervalId;

    onMount(async () => {
        services = await fetch_services();

        intervalId = setInterval(async () => {
            const currentServices = services;
            if (currentServices) {
                for (const s of currentServices) {
                    const is_running = await check_service(s.uuid);
                    services = services.map((item) =>
                        item.uuid === s.uuid
                            ? { ...item, running: is_running }
                            : item,
                    );
                }
            }
        }, 2000);
    });

    onDestroy(() => {
        clearInterval(intervalId);
    });
</script>

<div class="Service">
    {#each services as service (service.uuid)}
        <div class="item">
            <section class="info">
                <section style="display: flex; align-items: center;">
                    <div
                        class="indicator"
                        class:true={service.running}
                        class:false={!service.running}
                    ></div>
                    <span class="big">{service.name}</span>
                </section>
                <span>{service.path}</span>
            </section>

            <section class="buttons">
                <button
                    class="button true"
                    onclick={() =>
                        start_service(
                            service.uuid,
                            service.name,
                            service.path,
                            service.command,
                        )}
                >
                    Start Service
                </button>

                <button
                    class="button false"
                    onclick={() =>
                        stop_service(
                            service.uuid,
                            service.name,
                            service.path,
                            service.command,
                        )}
                >
                    Stop Service
                </button>

                <button
                    class="button false"
                    onclick={() =>
                        delete_service(
                            service.uuid,
                            service.name,
                            service.path,
                            service.command,
                        )}
                >
                    Delete Service
                </button>
            </section>
        </div>
    {/each}
</div>

<style>
    .Service {
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        justify-content: center;
        gap: var(--gap-2);
        @media (max-width: 768px) { padding: 0 var(--gap-3); }
        
        .item {
            background-color: var(--card-background);
            padding: var(--padding);
            border-radius: var(--radius);
            border: 2px solid var(--card-border);
            display: flex;
            flex-direction: column;
            gap: var(--gap);
            box-shadow: 0 5px 0 var(--card-border);
    
            .info {
                display: flex;
                flex-direction: column;
                gap: var(--gap);
            }
    
            .buttons {
                display: flex;
                gap: var(--gap);
            }
        }
    }
</style>