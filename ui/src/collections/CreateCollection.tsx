import { useMutation } from "react-query"
import { useNavigate } from 'react-router-dom'
import { useForm } from 'react-hook-form'
import { createCollection } from "../api"

export default function CreateCollection() {
  let navigate = useNavigate();
  const mutation = useMutation(col => createCollection(col), {
    onSuccess: (data) => {
      // TODO: navigate to collection page
      console.log(data)
      data.json().then(collection => {
        navigate(`/collections/${collection.id}`)
      })
    }
  })

  const { register, handleSubmit } = useForm()

  function onSubmit(formValues) {
    mutation.mutate(formValues)
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <div>
        <label htmlFor='name'>Name: </label>
        <input {...register('name')} />
      </div>
      <div>
        <label htmlFor='schema'>Schema: </label>
        <textarea {...register('schema')}></textarea>
      </div>
      <button>Submit</button>
    </form>
  )
}