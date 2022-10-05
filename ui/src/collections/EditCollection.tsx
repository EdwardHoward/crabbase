import { useMutation, useQuery } from 'react-query'
import { Link, useNavigate, useParams } from 'react-router-dom'
import { getCollection, updateCollection } from '../api'
import CollectionForm from './CollectionForm'
import { CollectionModel } from './CollectionModel'

export default function EditCollection() {
  const navigate = useNavigate()

  const { id = '' } = useParams()
  const { data, status } = useQuery(['collections', id], () => getCollection(id))

  const mutation = useMutation(col => updateCollection(id, col), {
    onSuccess: () => {
      navigate(`/collections/${id}`)
    }
  })

  function onSubmit(formValues: CollectionModel) {
    mutation.mutate({
      ...data,
      ...formValues
    })
  }

  if (status === 'loading') {
    return null
  }

  return (
    <>
      <div>{data.name}</div>
      <Link to={`/collections/${id}`}>Back</Link>
      <CollectionForm
        collection={data}
        onSubmit={onSubmit}
      />
    </>
  )
}