import { useQuery } from 'react-query'
import { Link, useParams } from 'react-router-dom'
import { getCollection } from '../api'

export default function Collection() {
  const { id = '' } = useParams()
  const { data, status } = useQuery(['collections', id], () => getCollection(id))

  if (status === 'loading') {
    return null
  }

  return (
    <div>
      <h3>{data.name}</h3>
      <code>{data.schema}</code>
      <Link to={`/collections/${id}/edit`}>Edit</Link>
    </div>
  )
}