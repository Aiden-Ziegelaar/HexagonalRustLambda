import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Update Product in Cart', function () {
    it('should add a product to a users cart then update it', async function () {
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

        let cart_item = {
            product_id: product_res.data.id,
            quantity: faker.number.int({
                min: 1, 
                max: 10
            })
        }

        //act
        let res = await axios.post(`${process.env.INF_API_ENDPOINT}main/cart/${user.username}/item`, cart_item)

        let res_update = await axios.patch(`${process.env.INF_API_ENDPOINT}main/cart/${user.username}/item/${cart_item.product_id}`, {
            quantity: 2
        })

        let res_get = await axios.get(`${process.env.INF_API_ENDPOINT}main/cart/${user.username}`)

        //assert
        assert.equal(res.status, 201)
        expect(res.data).to.include(cart_item)
        assert.equal(res_update.status, 200)
        assert.equal(res_get.status, 200)
        assert.equal(res_get.data[0].quantity, 2)
    })
});