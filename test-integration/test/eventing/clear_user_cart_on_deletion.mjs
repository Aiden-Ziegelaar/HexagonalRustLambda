import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

let completion_NFR = 10000;
let sleep = 500;
let iterations = completion_NFR / sleep;

describe('Clear Cart on User Deletion', function () {
    it('should add multiple products to a users cart then clear the cart when the user is deleted', async function () {
        //arrange
        this.timeout(completion_NFR);
        let user_id = faker.internet.userName();

        let cart_items = Array(10).fill().map(() => {
                return {
                    product_id: faker.string.uuid(),
                    quantity: faker.number.int({
                        min: 1, 
                        max: 10
                    })
                }
            })

        //act
        let res_user_create = await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email(),
            username: user_id,
        })

        let res_post_promises = cart_items.map(
            cart_item => axios.post(`${process.env.INF_API_ENDPOINT}main/cart/${user_id}/item`, cart_item)
        )
        await Promise.all(res_post_promises);

        let res_user_delete = await axios.delete(`${process.env.INF_API_ENDPOINT}main/user/${user_id}`)
        let cart_items_len = 10;
        let calls = 0;

        while (cart_items_len > 0 && calls < iterations) {
            let res_items_post_delete = await axios.get(`${process.env.INF_API_ENDPOINT}main/cart/${user_id}`)
            cart_items_len = res_items_post_delete.data.length;
            if (cart_items_len > 0) {
                calls++;
                await new Promise(resolve => setTimeout(resolve, sleep));
            }
        }

        //assert
        assert.equal(cart_items_len, 0);
        assert.equal(res_user_create.status, 201);
        assert.equal(res_user_delete.status, 200);
    })
});