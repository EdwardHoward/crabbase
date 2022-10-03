import { useQuery } from 'react-query'
import { useParams } from 'react-router-dom'

function getCollection(id: string) {
  return fetch(`http://localhost:8080/api/collections/${id}`)
    .then(res => res.json())
}

export default function Collection() {
  const { id = '' } = useParams()
  const { data, status } = useQuery(['collections', id], () => getCollection(id))

  if (status === 'loading') {
    return null
  }

  return (
    <div>
      {data.name}
    </div>
  )
}