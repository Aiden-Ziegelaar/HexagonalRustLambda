import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Clear Cart', function () {
    it('should add multiple products to a users cart then clear the cart', async function () {
        this.timeout(5000);
        //arrange
        let user_id = faker.internet.email();

        let cart_items = Array(10).fill().map(() => {
                return {
                    product_id: faker.string.uuid(),
                    user_id,
                    quantity: faker.number.int({
                        min: 1, 
                        max: 10
                    })
                }
            })

        //act
        let res_post_promises = cart_items.map(
            cart_item => axios.post(`${process.env.INF_API_ENDPOINT}main/cart/item`, cart_item)
        )
        await Promise.all(res_post_promises);

        let res_items_pre_delete = await axios.get(`${process.env.INF_API_ENDPOINT}main/cart`, {
            params: {
                id: user_id
            }
        })

        let res_delete = await axios.delete(`${process.env.INF_API_ENDPOINT}main/cart`, {
            params: {
                id: user_id
            }
        })

        let res_items_post_delete = await axios.get(`${process.env.INF_API_ENDPOINT}main/cart`, {
            params: {
                id: user_id
            }
        })

        //assert
        assert.equal(res_delete.status, 200);
        assert.equal(res_items_pre_delete.data.length, 10);
        assert.equal(res_items_post_delete.data.length, 0);
    })
});