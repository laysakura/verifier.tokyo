import init, { fetch_content, decode_and_verify_vc } from './pkg/verifier_tokyo';

async function fetchContent() {
    try {
        await init();
        const content = await fetch_content();
        document.getElementById('content').innerText = content;
    } catch (error) {
        console.error('Fetch error:', error);
        document.getElementById('content').innerText = 'Failed to load content';
    }
}

fetchContent();
