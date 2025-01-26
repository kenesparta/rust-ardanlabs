// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="intro.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="rest_service.html"><strong aria-hidden="true">2.</strong> REST Service</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="simple_http_server.html"><strong aria-hidden="true">2.1.</strong> Minimal HTTP Server</a></li><li class="chapter-item expanded "><a href="axum_hyper_tower_tokio.html"><strong aria-hidden="true">2.2.</strong> Service Stack</a></li><li class="chapter-item expanded "><a href="axum_extractors.html"><strong aria-hidden="true">2.3.</strong> Extractors</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="axum_extractors_path.html"><strong aria-hidden="true">2.3.1.</strong> Path Extraction</a></li><li class="chapter-item expanded "><a href="axum_extractors_query.html"><strong aria-hidden="true">2.3.2.</strong> Query Extraction</a></li><li class="chapter-item expanded "><a href="axum_extractors_headers.html"><strong aria-hidden="true">2.3.3.</strong> Header Extraction</a></li><li class="chapter-item expanded "><a href="axum_extractors_more.html"><strong aria-hidden="true">2.3.4.</strong> More Extractors</a></li></ol></li><li class="chapter-item expanded "><a href="simple_tower_layer.html"><strong aria-hidden="true">2.4.</strong> Add a Simple Tower Layer (State)</a></li><li class="chapter-item expanded "><a href="simple_tower_layer_mut.html"><strong aria-hidden="true">2.5.</strong> Add a Simple Tower Layer (Mutable State)</a></li><li class="chapter-item expanded "><a href="simple_tower_layer_multi_state.html"><strong aria-hidden="true">2.6.</strong> Multiple States - Extension Layers</a></li><li class="chapter-item expanded "><a href="simple_tower_recap.html"><strong aria-hidden="true">2.7.</strong> Quick Recap on State and Layers</a></li><li class="chapter-item expanded "><a href="nesting.html"><strong aria-hidden="true">2.8.</strong> Nesting Multiple Routers</a></li><li class="chapter-item expanded "><a href="nesting_state.html"><strong aria-hidden="true">2.9.</strong> Nested Routers with State</a></li><li class="chapter-item expanded "><a href="calling_other_services.html"><strong aria-hidden="true">2.10.</strong> Calling Other Services with Hyper</a></li><li class="chapter-item expanded "><a href="status_codes.html"><strong aria-hidden="true">2.11.</strong> Returning Status Codes</a></li><li class="chapter-item expanded "><a href="into_response.html"><strong aria-hidden="true">2.12.</strong> Using IntoResponse</a></li><li class="chapter-item expanded "><a href="error_handling.html"><strong aria-hidden="true">2.13.</strong> Error Handling with IntoResponse</a></li><li class="chapter-item expanded "><a href="error_handling_recap.html"><strong aria-hidden="true">2.14.</strong> Quick Recap on Nesting, Making Calls and Responses</a></li><li class="chapter-item expanded "><a href="static_content.html"><strong aria-hidden="true">2.15.</strong> Serving Static Content with Tower</a></li><li class="chapter-item expanded "><a href="header_auth1.html"><strong aria-hidden="true">2.16.</strong> Simple Header-Based Authentication</a></li><li class="chapter-item expanded "><a href="header_auth2.html"><strong aria-hidden="true">2.17.</strong> Simple Header-Based Auth with Middleware</a></li><li class="chapter-item expanded "><a href="header_auth3.html"><strong aria-hidden="true">2.18.</strong> Middleware Auth with Injection</a></li><li class="chapter-item expanded "><a href="layer_targets.html"><strong aria-hidden="true">2.19.</strong> Selectively Applying Layers</a></li><li class="chapter-item expanded "><a href="layer_router.html"><strong aria-hidden="true">2.20.</strong> Router Layers</a></li><li class="chapter-item expanded "><a href="layer_recap.html"><strong aria-hidden="true">2.21.</strong> Layer Recap</a></li></ol></li><li class="chapter-item expanded "><a href="axum_tracing.html"><strong aria-hidden="true">3.</strong> Tracing</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="axum_tracing_minimal.html"><strong aria-hidden="true">3.1.</strong> Minimal Example</a></li><li class="chapter-item expanded "><a href="axum_tracing_tower.html"><strong aria-hidden="true">3.2.</strong> Logging Axum/Tower</a></li><li class="chapter-item expanded "><a href="axum_spans_own.html"><strong aria-hidden="true">3.3.</strong> Timing Spans</a></li><li class="chapter-item expanded "><a href="axum_spans.html"><strong aria-hidden="true">3.4.</strong> Axum Spans</a></li><li class="chapter-item expanded "><a href="axum_file_log.html"><strong aria-hidden="true">3.5.</strong> Logging to a File</a></li><li class="chapter-item expanded "><a href="axum_log_json.html"><strong aria-hidden="true">3.6.</strong> Structured Logging to JSON</a></li><li class="chapter-item expanded "><a href="otel_top.html"><strong aria-hidden="true">3.7.</strong> OpenTelemetry</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="otel_hello.html"><strong aria-hidden="true">3.7.1.</strong> Hello Telemetry</a></li></ol></li></ol></li><li class="chapter-item expanded "><a href="openapi.html"><strong aria-hidden="true">4.</strong> OpenAPI Documentation</a></li><li class="chapter-item expanded "><a href="config_top.html"><strong aria-hidden="true">5.</strong> Handling Service Configuration</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="config_dot_env.html"><strong aria-hidden="true">5.1.</strong> Environment Variables with .env</a></li><li class="chapter-item expanded "><a href="config_crate.html"><strong aria-hidden="true">5.2.</strong> The Config Crate - Basics</a></li><li class="chapter-item expanded "><a href="config_http.html"><strong aria-hidden="true">5.3.</strong> Loading Config via HTTP</a></li><li class="chapter-item expanded "><a href="config_clap.html"><strong aria-hidden="true">5.4.</strong> CLI configuration with Clap</a></li><li class="chapter-item expanded "><a href="config_recap1.html"><strong aria-hidden="true">5.5.</strong> Recap</a></li></ol></li><li class="chapter-item expanded "><a href="grpc_intro.html"><strong aria-hidden="true">6.</strong> gRPC</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="grpc_hello.html"><strong aria-hidden="true">6.1.</strong> Hello Tonic - Protocol Definition</a></li><li class="chapter-item expanded "><a href="grpc_hello2.html"><strong aria-hidden="true">6.2.</strong> Hello Tonic - Project Definition and Build</a></li><li class="chapter-item expanded "><a href="grpc_hello3.html"><strong aria-hidden="true">6.3.</strong> Hello Tonic - The Server</a></li><li class="chapter-item expanded "><a href="grpc_hello4.html"><strong aria-hidden="true">6.4.</strong> Hello Tonic - The Client</a></li><li class="chapter-item expanded "><a href="grpc_stream.html"><strong aria-hidden="true">6.5.</strong> gRPC Streaming</a></li><li class="chapter-item expanded "><a href="grpc_stream2.html"><strong aria-hidden="true">6.6.</strong> gRPC Streaming - Protocol Definition</a></li><li class="chapter-item expanded "><a href="grpc_stream3.html"><strong aria-hidden="true">6.7.</strong> gRPC Streaming - The Server</a></li><li class="chapter-item expanded "><a href="grpc_stream4.html"><strong aria-hidden="true">6.8.</strong> gRPC Streaming - The Client</a></li><li class="chapter-item expanded "><a href="grpc_recap1.html"><strong aria-hidden="true">6.9.</strong> Recap So Far</a></li><li class="chapter-item expanded "><a href="grpc_auth.html"><strong aria-hidden="true">6.10.</strong> Authentication</a></li><li class="chapter-item expanded "><a href="grpc_tracing.html"><strong aria-hidden="true">6.11.</strong> Tracing</a></li><li class="chapter-item expanded "><a href="grpc_conclude.html"><strong aria-hidden="true">6.12.</strong> When to use gRPC</a></li></ol></li><li class="chapter-item expanded "><a href="ws_intro.html"><strong aria-hidden="true">7.</strong> Web Sockets</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="ws_echo_server.html"><strong aria-hidden="true">7.1.</strong> Minimal Echo Server</a></li><li class="chapter-item expanded "><a href="ws_client.html"><strong aria-hidden="true">7.2.</strong> A native WS client</a></li><li class="chapter-item expanded "><a href="ws_json.html"><strong aria-hidden="true">7.3.</strong> JSON</a></li></ol></li><li class="chapter-item expanded "><a href="svc_deploy_intro.html"><strong aria-hidden="true">8.</strong> Service Deployment</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="svc_deploy_test.html"><strong aria-hidden="true">8.1.</strong> Test Service</a></li><li class="chapter-item expanded "><a href="svc_deploy_native.html"><strong aria-hidden="true">8.2.</strong> Native Host Deployment</a></li><li class="chapter-item expanded "><a href="svc_deploy_docker.html"><strong aria-hidden="true">8.3.</strong> Docker Deployment</a></li></ol></li><li class="chapter-item expanded "><a href="design_intro.html"><strong aria-hidden="true">9.</strong> Service Design</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="design_company_arch.html"><strong aria-hidden="true">9.1.</strong> Understanding Your Company Architecture</a></li><li class="chapter-item expanded "><a href="design_svc_intro.html"><strong aria-hidden="true">9.2.</strong> Designing Individual Services</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="design_svc_layout.html"><strong aria-hidden="true">9.2.1.</strong> Layout</a></li><li class="chapter-item expanded "><a href="design_svc_config.html"><strong aria-hidden="true">9.2.2.</strong> Per-Service Configuration</a></li><li class="chapter-item expanded "><a href="design_svc_db.html"><strong aria-hidden="true">9.2.3.</strong> Per-Service Database</a></li><li class="chapter-item expanded "><a href="design_svc_layer.html"><strong aria-hidden="true">9.2.4.</strong> Layers and API Systems for Other Systems</a></li><li class="chapter-item expanded "><a href="design_svc_module.html"><strong aria-hidden="true">9.2.5.</strong> Finally, the service module itself</a></li></ol></li><li class="chapter-item expanded "><a href="design_svc_modular_monolith.html"><strong aria-hidden="true">9.3.</strong> Combining Services into a Modular Monolith</a></li><li class="chapter-item expanded "><a href="design_svc_exposure.html"><strong aria-hidden="true">9.4.</strong> Service Exposure</a></li><li class="chapter-item expanded "><a href="design_svc_scaling_out.html"><strong aria-hidden="true">9.5.</strong> Scaling Out</a></li></ol></li><li class="chapter-item expanded "><a href="wrap.html"><strong aria-hidden="true">10.</strong> Wrap Up</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString();
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
