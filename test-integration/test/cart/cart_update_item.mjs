import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Update Product in Cart', function () {
    it('should add a product to a users cart then update it', async function () {
        //arrange
        let user_id = faker.internet.userName();

        let cart_item = {
            product_id: faker.string.uuid(),
            quantity: 1
        }

        //act
        let res = await axios.post(`${process.env.INF_API_ENDPOINT}main/cart/${user_id}/item`, cart_item)

        let res_update = await axios.patch(`${process.env.INF_API_ENDPOINT}main/cart/${user_id}/item/${cart_item.product_id}`, {
            quantity: 2
        })

        let res_get = await axios.get(`${process.env.INF_API_ENDPOINT}main/cart/${user_id}`)

        //assert
        assert.equal(res.status, 201)
        expect(res.data).to.include(cart_item)
        assert.equal(res_update.status, 200)
        assert.equal(res_get.status, 200)
        assert.equal(res_get.data[0].quantity, 2)
    })
});