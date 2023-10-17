import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Remove Product from Cart', function () {
    it('should add a product to a users cart then remove it', async function () {
        //arrange
        let cart_item = {
            product_id: faker.string.uuid(),
            user_id: faker.internet.email(),
            quantity: faker.number.int({
                min: 1, 
                max: 10
            })
        }

        //act
        let res_add = await axios.post(`${process.env.INF_API_ENDPOINT}main/cart/item`, cart_item).catch(err => {
            console.log(err)
        })

        let res_delete = await axios.delete(`${process.env.INF_API_ENDPOINT}main/cart/item`, {
            params:{
                product_id: cart_item.product_id,
                email: cart_item.user_id
            }
        })

        let res_get = await axios.get(`${process.env.INF_API_ENDPOINT}main/cart`, {
            params: {
                id: cart_item.user_id
            }
        })

        //assert
        assert.equal(res_add.status, 201)
        expect(res_add.data).to.include(cart_item)
        assert.equal(res_delete.status, 200)
        assert.equal(res_get.status, 200)
        assert.equal(res_get.data.length, 0)
    })
});