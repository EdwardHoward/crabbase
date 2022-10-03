const baseUrl = "http://localhost:8080/api/collections"

export function getCollection(id: string) {
  return fetch(`${baseUrl}/${id}`)
    .then(res => res.json())
}

export function getCollections() {
  return fetch(`${baseUrl}`)
    .then(res => res.json())
}

export function updateCollection(id: string, collection) {
  console.log(id, collection)
  return fetch(`${baseUrl}/${id}`, {
    method: 'put',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(collection)
  })
}

export function createCollection(collection) {
  return fetch(`${baseUrl}`, {
    method: 'post',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(collection)
  })
}