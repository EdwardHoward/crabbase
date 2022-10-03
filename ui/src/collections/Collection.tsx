import { useQuery } from 'react-query'
import { useParams } from 'react-router-dom'
import { getCollection } from '../api'
import EditCollection from './EditCollection'

export default function Collection() {
  const { id = '' } = useParams()
  const { data, status } = useQuery(['collections', id], () => getCollection(id))

  if (status === 'loading') {
    return null
  }

  return (
    <div>
      <h3>{data.name}</h3>
      <EditCollection collection={data} />
    </div>
  )
}