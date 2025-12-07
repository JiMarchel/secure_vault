import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/verification/otp/')({
  component: RouteComponent,
})

function RouteComponent() {
  return <div>Hello "/verification/otp/"!</div>
}
