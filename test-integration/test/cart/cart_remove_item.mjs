import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Remove Product from Cart', function () {
    it('should add a product to a users cart then remove it', async function () {
        //arrange
        let user_id = faker.internet.userName();

        let cart_item = {
            product_id: faker.string.uuid(),
            quantity: faker.number.int({
                min: 1, 
                max: 10
            })
        }

        //act
        let res_add = await axios.post(`${process.env.INF_API_ENDPOINT}main/cart/${user_id}/item`, cart_item)

        let res_delete = await axios.delete(`${process.env.INF_API_ENDPOINT}main/cart/${user_id}/item/${cart_item.product_id}`)

        let res_get = await axios.get(`${process.env.INF_API_ENDPOINT}main/cart/${user_id}`)

        //assert
        assert.equal(res_add.status, 201)
        expect(res_add.data).to.include(cart_item)
        assert.equal(res_delete.status, 200)
        assert.equal(res_get.status, 200)
        assert.equal(res_get.data.length, 0)
    })
});