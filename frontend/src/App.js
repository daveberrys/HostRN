import html from "solid-js/html";
import Service from "./components/Service.js";
import CreateService from "./components/CreateService.js";
import Footer from "./components/Footer.js";

function App() {
    return html`
        <div class="index">
            <section class="main">
                <${CreateService} />
                <${Service} />
            </section>
            <${Footer} />
        </div>
    `;
}

export default App;
