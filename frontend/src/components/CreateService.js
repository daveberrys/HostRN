import html from 'solid-js/html';
import { save_service } from '../API.js';
import { createStore } from 'solid-js/store';

function CreateService() {
    const [form, setForm] = createStore({ 
        name: '', 
        path: '', 
        command: '' 
    });
    return html`
        <div class="CreateService">
            <section class="inputs">
                <input type="text" oninput=${(e) => setForm({ name: e.target.value })} placeholder="Service Name" />
                <input type="text" oninput=${(e) => setForm({ path: e.target.value })} placeholder="Service Path" />
                <input type="text" oninput=${(e) => setForm({ command: e.target.value })} placeholder="Service Command" />
            </section>

            <section class="buttons">
                <button
                    class="true"
                    onclick=${() => save_service(form.name, form.path, form.command)}
                >
                    Save Service
                </button>
            </section>
        </div>
    `;
}

export default CreateService;