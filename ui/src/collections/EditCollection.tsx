import { useMutation, useQuery, useQueryClient } from "react-query"
import { useForm } from 'react-hook-form'
import { getCollection, updateCollection } from "../api"
import { Link, useNavigate, useParams } from "react-router-dom"

export default function EditCollection() {
  const navigate = useNavigate()

  const { id = '' } = useParams()
  const { data, status } = useQuery(['collections', id], () => getCollection(id))

  const mutation = useMutation(col => updateCollection(id, col), {
    onSuccess: () => {
      navigate(`/collections/${id}`)
    }
  })

  const { register, handleSubmit } = useForm()

  function onSubmit(formValues) {
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
      <div>
        <Link to={`/collections/${id}`}>Back</Link>
      </div>
      <form onSubmit={handleSubmit(onSubmit)}>
        <div>
          <label htmlFor='name'>Name: </label>
          <input defaultValue={data.name} {...register('name')} />
        </div>
        <div>
          <label htmlFor='schema'>Schema: </label>
          <textarea defaultValue={data.schema} {...register('schema')}></textarea>
        </div>
        <button>Submit</button>
      </form>
    </>
  )
}