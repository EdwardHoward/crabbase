import { useMutation } from 'react-query'
import { useNavigate } from 'react-router-dom'
import { createCollection } from '../api'
import CollectionForm from './CollectionForm';

export default function CreateCollection() {
  const navigate = useNavigate();

  const mutation = useMutation(col => createCollection(col), {
    onSuccess: (data) => {
      data.json().then(collection => {
        navigate(`/collections/${collection.id}`)
      })
    }
  })

  function onSubmit(formValues) {
    mutation.mutate(formValues)
  }

  return (
    <CollectionForm
      onSubmit={onSubmit}
    />
  )
}