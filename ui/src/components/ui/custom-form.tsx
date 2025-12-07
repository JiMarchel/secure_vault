import { createFormHook, createFormHookContexts } from '@tanstack/react-form'
import {
  Field,
  FieldContent,
  FieldDescription,
  FieldError,
  FieldLabel,
} from './field'
import { Input } from './input'
import type { ReactNode } from 'react'

const { fieldContext, formContext, useFieldContext, useFormContext } =
  createFormHookContexts()

type FormControlProps = {
  label: string
  description?: string
}

type FormBaseProps = FormControlProps & {
  children: ReactNode
  horizontal?: boolean
  controlFirst?: boolean
}

export function FormBase({
  children,
  label,
  description,
  controlFirst,
  horizontal,
}: FormBaseProps) {
  const field = useFieldContext()
  const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid
  const labelElement = (
    <>
      <FieldLabel htmlFor={field.name}>{label}</FieldLabel>
      {description && <FieldDescription>{description}</FieldDescription>}
    </>
  )
  const errorElem = isInvalid && <FieldError errors={field.state.meta.errors} />

  return (
    <Field
      data-invalid={isInvalid}
      orientation={horizontal ? 'horizontal' : undefined}
    >
      {controlFirst ? (
        <>
          {children}
          <FieldContent>
            {labelElement}
            {errorElem}
          </FieldContent>
        </>
      ) : (
        <>
          <FieldContent>{labelElement}</FieldContent>
          {children}
          {errorElem}
        </>
      )}
    </Field>
  )
}

export function FormInput(props: FormControlProps) {
  const field = useFieldContext<string>()
  const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid

  return (
    <FormBase {...props}>
      <Input
        id={field.name}
        name={field.name}
        value={field.state.value}
        onBlur={field.handleBlur}
        onChange={(e) => field.handleChange(e.target.value)}
        aria-invalid={isInvalid}
        placeholder={`Enter your ${props.label.toLocaleLowerCase()}`}
      />
    </FormBase>
  )
}

const { useAppForm } = createFormHook({
  fieldComponents: {
    Input: FormInput,
  },
  formComponents: {},
  fieldContext,
  formContext,
})

export { useAppForm, useFieldContext, useFormContext }
