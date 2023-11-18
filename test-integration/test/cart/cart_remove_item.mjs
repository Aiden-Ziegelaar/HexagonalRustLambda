import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Remove Product from Cart', function () {
    it('should add a product to a users cart then remove it', async function () {
        //arrange
        let user = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email().toLowerCase(),
            username: faker.internet.userName(),
        }

        //arrange
        let product = {
            product_name: faker.commerce.productName(),
            description: faker.commerce.productDescription(),
            price_cents: Number(faker.commerce.price({
                dec: 0
            }))
        }

        let product_res = await axios.post(`${process.env.INF_API_ENDPOINT}main/product`, product)
        let user_res = await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user)

        let product_get = await axios.get(`${process.env.INF_API_ENDPOINT}main/product/${product_res.data.id}`)
        let user_get = await axios.get(`${process.env.INF_API_ENDPOINT}main/user/${user_res.data.username}`)

        let cart_item = {
            product_id: product_get.data.id,
            quantity: faker.number.int({
                min: 1, 
                max: 10
            })
        }

        //act
        let res_add = await axios.post(`${process.env.INF_API_ENDPOINT}main/cart/${user_get.data.username}/item`, cart_item)

        let res_delete = await axios.delete(`${process.env.INF_API_ENDPOINT}main/cart/${user_get.data.username}/item/${cart_item.product_id}`)

        let res_get = await axios.get(`${process.env.INF_API_ENDPOINT}main/cart/${user_get.data.username}`)

        //assert
        assert.equal(res_add.status, 201)
        expect(res_add.data).to.include(cart_item)
        assert.equal(res_delete.status, 200)
        assert.equal(res_get.status, 200)
        assert.equal(res_get.data.length, 0)
    })
});