async function fetchContent() {
    try {
        const response = await fetch('https://example.com/');
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }
        const text = await response.text();
        document.getElementById('content').innerText = text;
    } catch (error) {
        console.error('Fetch error:', error);
        document.getElementById('content').innerText = 'Failed to load content';
    }
}

fetchContent();
