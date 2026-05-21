import html from 'solid-js/html';
import { save_service } from '../API.js';
import { signal } from 'solid-js';

function CreateService() {
    const services = signal({ name: '', path: '', command: '' });
    return html`
        <div class="CreateService">
            <section class="inputs">
                <input type="text" oninput=${(e) => services.name = e.target.value} placeholder="Service Name" />
                <input type="text" oninput=${(e) => services.path = e.target.value} placeholder="Service Path" />
                <input type="text" oninput=${(e) => services.command = e.target.value} placeholder="Service Command" />
            </section>

            <section class="buttons">
                <button
                    class="true"
                    onclick=${() => save_service(services.name, services.path, services.command)}
                >
                    Save Service
                </button>
            </section>
        </div>
    `;
}

export default CreateService;