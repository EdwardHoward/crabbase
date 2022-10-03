import { useMutation, useQueryClient } from "react-query"
import { useForm } from 'react-hook-form'
import { updateCollection } from "../api"

export default function EditCollection({
  collection
}: {
  collection: { id: string, name: string, schema: string }
}) {
  const queryClient = useQueryClient()
  const mutation = useMutation(col => updateCollection(collection.id, col), {
    onSuccess: () => {
      queryClient.invalidateQueries(['collections', collection.id])
    }
  })

  const { register, handleSubmit } = useForm()

  function onSubmit(formValues) {
    mutation.mutate({
      ...collection,
      ...formValues
    })
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <div>
        <label htmlFor='name'>Name: </label>
        <input defaultValue={collection.name} {...register('name')} />
      </div>
      <div>
        <label htmlFor='schema'>Schema: </label>
        <textarea defaultValue={collection.schema} {...register('schema')}></textarea>
      </div>
      <button>Submit</button>
    </form>
  )
}