import { useQuery } from 'react-query'
import { Link } from 'react-router-dom'

function getCollections() {
  return fetch("http://localhost:8080/api/collections")
    .then(res => res.json())
}

export default function Collections() {
  const { data, status } = useQuery('collections', getCollections)

  if (status === 'loading') {
    return null
  }

  return (
    <ul>
      {data && data.map((collection) => (
        <li key={collection.id}>
          <Link to={`/collections/${collection.id}`}>
            {collection.name}
          </Link>
        </li>
      ))}
    </ul>
  )
}