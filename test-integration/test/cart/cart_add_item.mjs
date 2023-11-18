import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Add Product to Cart', function () {
    it('should add a product to a users cart', async function () {
        this.timeout(10000);
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

        let user_res = await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user)
        let product_res = await axios.post(`${process.env.INF_API_ENDPOINT}main/product`, product)

        let product_get = await axios.get(`${process.env.INF_API_ENDPOINT}main/product/${product_res.data.id}`)
        let user_get = await axios.get(`${process.env.INF_API_ENDPOINT}main/user/${user.username}`)

        let cart_item = {
            product_id: product_get.data.id,
            quantity: faker.number.int({
                min: 1, 
                max: 10
            })
        }

        //act
        let res = await axios.post(`${process.env.INF_API_ENDPOINT}main/cart/${user_get.data.username}/item`, cart_item)

        //assert
        assert.equal(product_res.status, 201)
        assert.equal(user_res.status, 201)
        assert.equal(res.status, 201)
        expect(res.data).to.include(cart_item)
    })
});