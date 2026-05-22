import html from "solid-js/html";
import { For, createResource } from "solid-js";
import { fetch_services, start_service, delete_service, stop_service } from "../API.js";

function Service() {
    const [services] = createResource(fetch_services)
    return html`
        <div class="Service">
            <${For} each=${services}>
                ${service => html`
                    <div class="item">
                        <section class="info">
                            <span class="big">${service.name}</span>
                            <span>${service.path}</span>
                            <span class="small">${service.uuid}</span>
                        </section>

                        <section class="buttons">
                            <button
                                class="button true"
                                onclick=${() => start_service(service.uuid, service.name, service.path, service.command)}
                            >
                                Start Service
                            </button>

                            <button
                                class="button false"
                                onclick=${() => stop_service(service.uuid, service.name, service.path, service.command)}
                            >
                                Stop Service
                            </button>

                            <button
                                class="button false"
                                onclick=${() => delete_service(service.uuid, service.name, service.path, service.command)}
                            >
                                Delete Service
                            </button>
                        </section>
                    </div>
                `}
            </For>
        </div>
    `;
}

export default Service;
