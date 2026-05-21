import html from "solid-js/html";
import Service from "./components/Service.js"
import CreateService from "./components/CreateService.js"

function App() {
    return html`
        <div class="index">
            <${CreateService} />
            <${Service} />
        </div>
    `;
}

export default App;
