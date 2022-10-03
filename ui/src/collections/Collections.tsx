import { useQuery } from 'react-query'
import { Link } from 'react-router-dom'
import { getCollections } from '../api'

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