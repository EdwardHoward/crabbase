import { useForm } from 'react-hook-form'
import { CollectionModel } from './CollectionModel'

type CollectionFormProps = {
  collection?: CollectionModel,
  onSubmit: (formValues: CollectionModel) => void
}

export default function CollectionForm({ collection, onSubmit }: CollectionFormProps) {
  const { register, handleSubmit } = useForm<CollectionModel>({
    defaultValues: collection || {}
  })

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