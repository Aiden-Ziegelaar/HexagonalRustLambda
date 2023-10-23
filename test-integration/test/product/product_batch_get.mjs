import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Get Product Batch', function () {
    it('should get a batch of created products', async function () {
        //arrange
        let products = []
        for(let i = 0; i < 10; i++){
            products.push({
                product_name: faker.commerce.productName(),
                description: faker.commerce.productDescription(),
                price_cents: Number(faker.commerce.price({
                    dec: 0
                }))
            })
        }

        //act
        let res_post_promises = products.map(
            product => axios.post(`${process.env.INF_API_ENDPOINT}main/product`, product)
        )

        let res_post = await Promise.all(res_post_promises)

        let res_batch_get = await axios.get(`${process.env.INF_API_ENDPOINT}main/product`,
            {
                params: {
                    id: res_post.map(res => res.data.id)
                },
                paramsSerializer:{
                    arrayFormat: 'repeat',
                    indexes: null
                }
            }
        )

        //assert
        assert.equal(res_batch_get.status, 200)
        assert.equal(res_batch_get.data.products.length, 10)
    })

    it('should get a batch of created products and skip not found products', async function () {
        //arrange
        let products = []
        for(let i = 0; i < 10; i++){
            products.push({
                product_name: faker.commerce.productName(),
                description: faker.commerce.productDescription(),
                price_cents: Number(faker.commerce.price({
                    dec: 0
                }))
            })
        }

        //act
        let res_post_promises = products.map(
            product => axios.post(`${process.env.INF_API_ENDPOINT}main/product`, product)
        )

        let res_post = await Promise.all(res_post_promises)

        let res_batch_get = await axios.get(`${process.env.INF_API_ENDPOINT}main/product`,
            {
                params: {
                    id: res_post.map(res => res.data.id).concat(faker.string.uuid(), faker.string.uuid())
                },
                paramsSerializer:{
                    arrayFormat: 'repeat',
                    indexes: null
                }
            }
        )

        //assert
        assert.equal(res_batch_get.status, 200)
        assert.equal(res_batch_get.data.products.length, 10)
    })

    it('should return an empty vec on not found products', async function () {
        //arrange
        let products = []

        //act
        let res_batch_get = await axios.get(`${process.env.INF_API_ENDPOINT}main/product`,
        {
            params: {
                id: [faker.string.uuid(), faker.string.uuid()]
            },
            paramsSerializer:{
                arrayFormat: 'repeat',
                indexes: null
            }
        }
        )

        //assert
        assert.equal(res_batch_get.status, 200)
        assert.equal(res_batch_get.data.products.length, 0)
    })
});