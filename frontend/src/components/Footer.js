import html from "solid-js/html";

function Footer() {
    return html`
        <div class="Footer">
            <section class="flex column">
                <span class="big">HostRN</span>
                <span>A fast, and simple way to host your non-docker apps.</span>
            </section>

            <section>
                <a
                    href="https://github.com/daveberrys/HostRN"
                    target="_blank"
                >
                    GitHub
                </a>
            </section>
        </div>
    `;
}

export default Footer;
