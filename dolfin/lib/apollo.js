import {useMemo} from 'react'
import {ApolloClient, HttpLink, InMemoryCache} from '@apollo/client'
import {concatPagination} from '@apollo/client/utilities'
import merge from 'deepmerge'
import isEqual from 'lodash/isEqual'

let apolloClient;

function createApolloClient() {
    return new ApolloClient({
        ssrMode: typeof window === 'undefined',
        link: new HttpLink({
            uri: 'http://localhost:8080/api', // Server URL (must be absolute)
            credentials: 'same-origin', // Additional fetch() options like `credentials` or `headers`
        }),
        cache: new InMemoryCache({
            typePolicies: {
                Query: {
                    fields: {
                        allPosts: concatPagination(),
                    },
                },
            },
        }),
    })
}

export function initializeApollo(initialState = null) {
    const _apolloClient = apolloClient ?? createApolloClient()

    if (initialState) {
        const existingCache = _apolloClient.extract()

        const data = merge(initialState, existingCache, {
            arrayMerge: (destinationArray, sourceArray) => [
                ...sourceArray,
                ...destinationArray.filter((d) =>
                    sourceArray.every((s) => !isEqual(d, s))
                ),
            ],
        })

        _apolloClient.cache.restore(data)
    }
    if (typeof window === 'undefined') return _apolloClient
    if (!apolloClient) apolloClient = _apolloClient

    return _apolloClient
}

export function useApollo(initialState) {
    return useMemo(() => initializeApollo(initialState), [initialState])
}