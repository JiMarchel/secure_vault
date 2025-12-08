import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { toast } from 'sonner'
import { APIError } from '@/lib/error-handling'

export function getContext() {
  const queryClient = new QueryClient({
    defaultOptions: {
      mutations: {
        onError: (error: unknown) => {
          if (error instanceof APIError) {
            toast.error(error.errorResponse.error.message, {
              duration: 10000,
              description: error.errorResponse.error.details?.validationErrors?.map(
                (v) => <div key={v.field}>{v.message}</div>,
              ),
            })
          }
        }
      }
    }
  })
  return {
    queryClient,
  }
}

export function Provider({
  children,
  queryClient,
}: {
  children: React.ReactNode
  queryClient: QueryClient
}) {
  return (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  )
}
