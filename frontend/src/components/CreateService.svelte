<script>
    import { save_service } from "../API.js";

    let name = $state("");
    let path = $state("");
    let command = $state("");
</script>

<div class="CreateService">
    <section class="inputs">
        <input
            type="text"
            oninput={(e) => (name = e.target.value)}
            placeholder="Service Name"
        />
        <input
            type="text"
            oninput={(e) => (path = e.target.value)}
            placeholder="Service Path"
        />
        <input
            type="text"
            oninput={(e) => (command = e.target.value)}
            placeholder="Service Command"
        />
    </section>

    <section class="buttons">
        <button
            class="button true"
            onclick={() => save_service(name, path, command)}
        >
            Save Service
        </button>
    </section>
</div>

<style>
    .CreateService {
        background-color: var(--card-background);
        padding: var(--padding);
        display: flex;
        /*flex-direction: column;*/
        gap: var(--gap-2);
        box-shadow: 0 5px 0 var(--card-border);
        width: auto;
    
        .inputs {
            display: flex;
            flex-direction: column;
            gap: var(--gap-2);
            flex: 1;
        }
    
        .buttons {
            display: flex;
            gap: var(--gap);
        }
    }
</style>