<div class="loading" />

<style>
  .loading {
    position: relative;
    display: inline-block;
    width: 0.8em;
    height: 0.8em;
    background: var(--border-color);
    border-radius: 50%;
    animation: spin 1s infinite linear;
    transform-origin: 0 1em;
    vertical-align: middle;
    margin-bottom: 1.4em;
    margin-right: 0.5em;
  }

  .loading:before,
  .loading:after {
    content: '';
    position: absolute;
    display: inline-block;
    background: var(--border-color);
    border-radius: 50%;
    width: 0.8em;
    height: 0.8em;
    top: 0;
    left: 0;
    transform: rotate(120deg);
    transform-origin: 0em 1em;
  }

  .loading:after {
    transform: rotate(240deg);
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }
</style>
