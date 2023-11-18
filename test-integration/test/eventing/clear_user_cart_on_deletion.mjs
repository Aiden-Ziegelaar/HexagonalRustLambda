import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

let completion_NFR = 20000;
let sleep = 500;
let iterations = completion_NFR / sleep;

describe('Clear Cart on User Deletion', function () {
    it('should add multiple products to a users cart then clear the cart when the user is deleted', async function () {
        //arrange
        this.timeout(completion_NFR);
        this.timeout(10000);
        //arrange
        let user = {
            first: faker.person.firstName(),
            last: faker.person.lastName(),
            email: faker.internet.email().toLowerCase(),
            username: faker.internet.userName(),
        }

        //arrange
        let product = Array(10).fill().map(() => {
            return {
                product_name: faker.commerce.productName(),
                description: faker.commerce.productDescription(),
                price_cents: Number(faker.commerce.price({
                    dec: 0
                }))
            }
        })

        let user_res = await axios.post(`${process.env.INF_API_ENDPOINT}main/user`, user)
        let product_res = await Promise.all(product.map((product) => axios.post(`${process.env.INF_API_ENDPOINT}main/product`, product)))

        let cart_items = Array(10).fill().map((_value, index) => {
                return {
                    product_id: product_res[index].data.id,
                    quantity: faker.number.int({
                        min: 1, 
                        max: 10
                    })
                }
            })

        //act
        for(let cart_item of cart_items) {
            await axios.post(`${process.env.INF_API_ENDPOINT}main/cart/${user_res.data.username}/item`, cart_item)
            await axios.get(`${process.env.INF_API_ENDPOINT}main/cart/${user_res.data.username}`) // wait for strong consistency
        }

        let res_user_delete = await axios.delete(`${process.env.INF_API_ENDPOINT}main/user/${user_res.data.username}`)
        let cart_items_len = 10;
        let calls = 0;

        while (cart_items_len > 0 && calls < iterations) {
            let res_items_post_delete = await axios.get(`${process.env.INF_API_ENDPOINT}main/cart/${user_res.data.username}`)
            cart_items_len = res_items_post_delete.data.length;
            if (cart_items_len > 0) {
                calls++;
                await new Promise(resolve => setTimeout(resolve, sleep));
            }
        }

        //assert
        assert.equal(cart_items_len, 0);
        assert.equal(user_res.status, 201);
        assert.equal(res_user_delete.status, 200);
    })
});