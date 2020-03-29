import template from './files.pug'

addEventListener('load', () => {
  const domain = `http://${location.hostname}:8000`
  window.app = {
    domain,
    static: domain + '/static/',
  }
  list('')

  addEventListener('hashchange', e => {
    list(location.hash.slice(1))
  })
})

function list(path) {
  fetch(`${app.domain}/api/files/${path}`, {
    headers: {
      'Access-Control-Allow-Origin': app.domain
    }
  })
    .then((response) => {
      return response.json()
    })
    .then((data) => {
      document.querySelector("main").innerHTML = template({ metas: data })
    })
}
