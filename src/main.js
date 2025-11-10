const { invoke } = window.__TAURI__.core;
const btn = document.getElementById('fetchBtn');

let buttonclick = false;
function showLoading() {
  console.log("Debug");
  buttonclick = true;
  btn.disabled = true;
btn.innerHTML = '<div class="loader"></div> Loading...';
invoke('fetch_url');
}
document.getElementById('fetchBtn').addEventListener('click', showLoading);