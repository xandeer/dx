import template from './files.pug'

addEventListener('load', () => {
  const domain = location.origin
  window.app = {
    domain,
    static: domain + '/static/',
  }
  list('')

  addEventListener('hashchange', _ => {
    list(getHash())
  })

  const $upload = document.querySelector('#upload')
  $upload.addEventListener('change', _ => {
    upload($upload.files[0])
  })
})

function upload(file) {
  fetch(`${app.domain}/api/upload/${getHash()}/${file.name}`, {
    method: 'POST',
    body: file
  }).then(_ => {
    list(getHash())
    console.log(`File ${file.name} uploaded successfully.`)
  })
    .catch(error => console.log(error))
}

function getHash() {
  return location.hash.slice(1)
}

function list(path) {
  fetch(`${app.domain}/api/files/${path}`).then(response =>
    response.json()
  ).then(data =>
    document.querySelector('#files').innerHTML = template({ metas: data })
  ).catch(error => console.log(error))
}
