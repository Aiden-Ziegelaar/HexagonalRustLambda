import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

let completion_NFR = 5000;
let sleep = 500;
let iterations = completion_NFR / sleep;

describe('Remove Product from Cart on Product Deletion', function () {
    it('should add a product to multiple users cart then clear the cart when the product is deleted', async function () {
        //arrange
        this.timeout(completion_NFR);
        let user_id_1 = faker.internet.userName();
        let user_id_2 = faker.internet.userName();

        let product = {
            product_name: faker.commerce.productName(),
            description: faker.commerce.productDescription(),
            price_cents: Number(faker.commerce.price({
                dec: 0
            }))
        }

        //act
        let res_product_create = await axios.post(`${process.env.INF_API_ENDPOINT}main/product`, product)
                
        await Promise.all([
            axios.post(`${process.env.INF_API_ENDPOINT}main/cart/${user_id_1}/item/`, {
                product_id: res_product_create.data.id,
                quantity: 1
            }),
            axios.post(`${process.env.INF_API_ENDPOINT}main/cart/${user_id_2}/item`, {
                product_id: res_product_create.data.id,
                quantity: 1
            })
        ])

        let res_product_delete = await axios.delete(`${process.env.INF_API_ENDPOINT}main/product/${res_product_create.data.id}`)

        let cart_items_len = 2;
        let calls = 0;

        while (cart_items_len > 0 && calls < iterations) {
            let res_items_post_delete_u1 = await axios.get(`${process.env.INF_API_ENDPOINT}main/cart/${user_id_1}`)
            let res_items_post_delete_u2 = await axios.get(`${process.env.INF_API_ENDPOINT}main/cart/${user_id_2}`)
            cart_items_len = res_items_post_delete_u1.data.length + res_items_post_delete_u2.data.length;
            if (cart_items_len > 0) {
                calls++;
                await new Promise(resolve => setTimeout(resolve, sleep));
            }
        }

        //assert
        assert.equal(cart_items_len, 0);
        assert.equal(res_product_delete.status, 200);
    })
});